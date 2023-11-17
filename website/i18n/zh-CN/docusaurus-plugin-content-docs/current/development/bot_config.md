---
sidebar_position: 1
---

# 自定义机器人配置

可通过对源码中 `config/bot.json` 文件进行修改来自定义机器人配置。

## 机器人配置规则

以下列配置代码为例：

```json
{
  "id": "87cc7e88-fd55-4c01-9a34-d0f1397b1c73",
  "name": "Polestar Assistant",
  "desc": "Polestar Assistant is a AI assistant powered by OpenAI",
  "avatar": {
    "name": "🌉",
    "color": "#EDF7FBFF"
  },
  "cat": "Assistant",
  "tags": [
    "AI"
  ],
  "lang": [
    "en"
  ],
  "url": "https://api.openai.com/v1/chat/completions",
  "headers": {
    "Authorization": "{OpenAI}"
  },
  "params": {
    "model": "gpt-3.5-turbo",
    "prompt": ""
  }
}
```

这里详细介绍下每个字段的含义及配置注意事项：

1. **id** 机器人的唯一标识，这里采用 UUID 字符串，可通过工具生成。
2. **name** 机器人对外展示的名称。
3. **desc** 机器人对外展示的介绍信息，可为空。
4. **avatar** 机器人对外展示的头像信息，可以是文字、emoji、或者图片。
5. **cat** 机器人的所属类别，可为空。
6. **tags** 机器人的标签
7. **lang** 机器人对应的语言版本，可支持多版本。例如当前用户切换的版本为 `en`，如果预设机器人的 lang 字段不包含 `en`，那么就不会出现该机器人。
8. **url** 机器人发送请求的接口。
9. **headers** 机器人发送请求的请求头。
10. **params** 机器人发送请求的请求数据。

## Key 配置

Polestar 提供官方和自定义 Key 两种方式。使用的差异在上面一部分的机器人配置中，我们需要重点关注 `url`、`headers`、以及 `params` 3 个字段。

例如你想使用 OpenAI 的服务，那么 `url` 就配置为 OpenAI 的 API 地址，`headers` 需要按照 OpenAI 的 Headers 进行配置，`params` 参数也是如此。

重点需要关注下不同服务的 Key 配置。在 `bot.json` 中，有 `tokens` 字段，提供了不同平台定义 Key 的配置。

```json
{
  "tokens": {
    "OpenAI": "sk-xxx"
  },
  "bots": [
    {
      "id": "87cc7e88-fd55-4c01-9a34-d0f1397b1c73",
      // ...
      "headers": {
        "Authorization": "{OpenAI}"
      }
    },
    {
      "id" "93c55207-c9e5-4642-a745-a9b080ca391a",
      // ...
      "headers": {
        "Authorization": "${PolestarKey}"
      }
    }
  ]
}
```

项目会读取 `bot.json` 中标记 `{}` 中的 tokens 去定义中寻找并填充。如果是在程序运行过程中获取，会被标记为 `${}`，例如官方提供的服务的 Key，因为它会定期自动更新，所以采用 `${}` 在运行过程中进行读取。

## 小结

完成上述配置后，可以自行通过[打包](./package.md)
