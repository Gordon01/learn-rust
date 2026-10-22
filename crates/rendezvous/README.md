# Сервер для задачи [UDP hole punching](../../assignments/udp-hole.md)

## Установка

```bash
cd crates
cargo install --path rendezvous --locked
# Установка сервиса в systemd
sudo cp rendezvous/rendezvous.service /etc/systemd/system
sudo systemctl daemon-reload
sudo systemctl enable --now rendezvous
```

## Просмотр логов

```bash
journalctl -u rendezvous -f
```

Логи также доступны по HTTP, в виде стрима или последних записей.

## Обновление

```bash
# Замените бинарь
cargo install --path crates/rendezvous --locked
# Перезапустите сервис
sudo systemctl restart rendezvous
```
