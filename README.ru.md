# ReShip

**Языки:** [English](README.md) | Русский

ReShip — распределённая платформа доставки релизов и обновлений приложений. CI один раз публикует immutable-релиз; клиент через RSP определяет целевую версию, скачивает только недостающий контент, собирает чистый snapshot установки и атомарно активирует его.

> **Статус:** bootstrap / архитектура v0.1. Модель протокола и границы проекта уже заданы; публикация, persistence, SDK, региональная маршрутизация и activation будут реализовываться отдельными задачами `RS-*`.

## Основные свойства

- единый pipeline релиза от CI до клиента;
- immutable manifests и content-addressed artifacts;
- чистая установка: удалённый из релиза файл не может пережить обновление;
- file-level differential delivery с первой версии (`keep / download / delete`);
- CDN необязателен и не меняет клиентский протокол;
- региональное масштабирование с fallback на origin;
- семантика RSP не зависит от HTTP;
- сервер не знает ничего про ETS как особый продукт.

## Структура

```text
protocol/                 RSP — отдельный вложенный Rust workspace
  rsp-core                примитивы и версия протокола
  rsp-manifest            immutable snapshot установки
  rsp-discovery           desired state application/channel
  rsp-http                HTTP/CDN binding

crates/
  reship-server           control plane / origin API
  reship-storage          storage boundary
  reship-cli              CLI для CI и операторов

sdk/                      контракты и будущие SDK языков
```

Направление зависимостей строгое: ReShip и SDK могут зависеть от RSP; RSP никогда не зависит от `reship-*`.

Подробности — в [ARCHITECTURE.ru.md](ARCHITECTURE.ru.md).
