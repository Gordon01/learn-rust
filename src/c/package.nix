{ ... }: {
  perSystem = { pkgs, ... }: {
    packages.c = pkgs.stdenv.mkDerivation rec {
      pname = "stack-c";
      version = "0.1.0";
      src = ./.;

      buildPhase = ''
        $CC 6_stack.c -o 6_stack
      '';
      installPhase = ''
        mkdir -p $out/bin
        mv 6_stack $out/bin/${pname}
      '';
    };
  };
}
