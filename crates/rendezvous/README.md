# Сервер для задачи [UDP hole punching](../../assignments/udp-hole.md)

## Установка через Home Manager (Linux и macOS)

Flake экспортирует пакет `packages.<system>.rendezvous` и модуль
`homeModules.rendezvous`. Добавьте вход в свой flake:

```nix
inputs.cource.url = "git+https://gitverse.ru/sb-rs/cource.git";
```

Подключите модуль в конфигурации Home Manager (передайте `inputs` через
`extraSpecialArgs` или используйте его из области видимости вашего flake):

```nix
{ inputs, ... }: {
  imports = [ inputs.cource.homeModules.rendezvous ];

  services.rendezvous = {
    enable = true;
    # По умолчанию: HTTP TCP 8080, UDP 4200.
    httpPort = 8081;
    udpPort = 4201;
  };

}
```

### Linux

Запустите сервис:

```bash
systemctl --user start rendezvous
systemctl --user status rendezvous
```

Для запуска без входа в систему и работы
после выхода отдельно включите lingering (требуются права администратора):

```bash
sudo loginctl enable-linger "$USER"
```

### macOS

Home Manager устанавливает агент `org.nix-community.home.rendezvous` и запускает
его при активации и последующих входах в систему. Агент работает только во время
пользовательского сеанса: это не системный daemon для работы до входа или после
выхода. При неуспешном завершении launchd перезапускает процесс с ограничением
частоты запусков в 5 секунд.

Проверка состояния и перезапуск в графическом сеансе:

```bash
launchctl print "gui/$(id -u)/org.nix-community.home.rendezvous"
launchctl kickstart -k "gui/$(id -u)/org.nix-community.home.rendezvous"
```

stdout и stderr записываются в `~/Library/Logs/rendezvous.log`.

### Сборка пакета

Пакет также можно собрать отдельно из этого репозитория:

```bash
nix build .#rendezvous
```

## Установка вручную

```bash
cd crates
cargo install --path rendezvous --locked
# Установка сервиса в systemd
sudo cp rendezvous/rendezvous.service /etc/systemd/system
sudo systemctl daemon-reload
sudo systemctl enable --now rendezvous
```

### Настройка портов

Без параметров сервер использует HTTP TCP 8080 и UDP 4200. При прямом запуске
порты можно переопределить:

## Просмотр логов

Linux (Home Manager):

```bash
journalctl --user -u rendezvous -f
```

macOS:

```bash
tail -f ~/Library/Logs/rendezvous.log
```

На обеих платформах логи также доступны через HTTP API.
