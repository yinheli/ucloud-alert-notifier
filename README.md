# ucloud-alert-notifier

ucloud alert webhook for feishu

## Usage

### Run server

```bash
./ucloud-alert-notifier serve --help
```

```text
Run server to receive webhook

Usage: ucloud-alert-notifier serve [OPTIONS] --webhook <WEBHOOK> [BIND]

Arguments:
  [BIND]  bind [env: ALERT_NOTIFIER_BIND=] [default: 0.0.0.0:8080]

Options:
  -w, --webhook <WEBHOOK>  feishu webhook [env: ALERT_NOTIFIER_WEBHOOK=]
  -s, --secret <SECRET>    webhook secret [env: ALERT_NOTIFIER_SECRET=]
  -h, --help               Print help
```

### Add webhook

Refer: https://console.ucloud.cn/umon/contact

## Resources

- https://docs.ucloud.cn/umon/guide/webhook
- https://open.feishu.cn/document/ukTMukTMukTM/ucTM5YjL3ETO24yNxkjN?lang=zh-CN
