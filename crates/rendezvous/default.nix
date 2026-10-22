{ self, ... }: {
  perSystem = { pkgs, ... }: {
    packages.rendezvous = pkgs.rustPlatform.buildRustPackage {
      pname = "rendezvous";
      version = (fromTOML (builtins.readFile ./Cargo.toml)).package.version;
      src = ../.;
      cargoLock.lockFile = ../Cargo.lock;
      cargoBuildFlags = [ "-p" "rendezvous" ];
      cargoTestFlags = [ "-p" "rendezvous" ];
      meta.mainProgram = "rendezvous";
    };
  };

  flake.homeModules.rendezvous = { config, lib, pkgs, ... }:
    let
      cfg = config.services.rendezvous;
      package = self.packages.${pkgs.stdenv.hostPlatform.system}.rendezvous;
      command = [
        (lib.getExe package)
        "--http-port"
        (toString cfg.httpPort)
        "--udp-port"
        (toString cfg.udpPort)
      ];
      logDirectory = "${config.home.homeDirectory}/Library/Logs";
    in {
      options.services.rendezvous = {
        enable = lib.mkEnableOption "the rendezvous server";

        httpPort = lib.mkOption {
          type = lib.types.port;
          default = 8080;
          description =
            "TCP port for the HTTP API, bound to all IPv4 interfaces.";
        };

        udpPort = lib.mkOption {
          type = lib.types.port;
          default = 4200;
          description =
            "UDP port for rendezvous, bound to all IPv4 interfaces.";
        };
      };

      config = lib.mkIf cfg.enable {
        home.packages = [ package ];

        systemd.user.services.rendezvous =
          lib.mkIf pkgs.stdenv.hostPlatform.isLinux {
            Unit = {
              Description = "Rendezvous service";
              After = [ "network.target" ];
            };
            Service = {
              ExecStart = lib.escapeShellArgs command;
              Restart = "on-failure";
              RestartSec = 5;
              StandardOutput = "null";
              StandardError = "journal";
              NoNewPrivileges = true;
            };
            Install.WantedBy = [ "default.target" ];
          };

        launchd.agents.rendezvous = lib.mkIf pkgs.stdenv.hostPlatform.isDarwin {
          enable = true;
          config = {
            ProgramArguments = command;
            RunAtLoad = true;
            KeepAlive.SuccessfulExit = false;
            ThrottleInterval = 5;
            StandardOutPath = "${logDirectory}/rendezvous.log";
            StandardErrorPath = "${logDirectory}/rendezvous.log";
          };
        };
      };
    };
}
