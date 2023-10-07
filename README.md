<h1 align="center">@lambda-group/axum-js</h1>

<div align="center">

[![NPM version](https://img.shields.io/npm/v/@lambda-group/axum-js.svg?style=flat-square)](https://www.npmjs.com/package/@lambda-group/axum-js)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)](CONTRIBUTING.md)
[![Twitter Follow](https://img.shields.io/twitter/follow/lambda_group?style=social)](https://twitter.com/lambda_group)

</div>

`axum-js` is a TypeScript/JavaScript wrapper around the Rust [Axum](https://github.com/tokio-rs/axum) crate, brought to you via [Napi-rs](https://github.com/napi-rs/napi-rs). This initiative seeks to imbue the JavaScript ecosystem with the resiliency and efficiency inherent to Axum.

Initiated during Hacktoberfest 2023 by Daniel Boll, this project now thrives under the stewardship of the `@lambda-group`.

## 🎯 Supported Platforms

- `darwin-x64`
- `linux-x64-gnu`
- `win32-x64-msvc`

## 🚀 Installation

```bash
npm install @lambda-group/axum-js
```

## 🛠 Usage

```javascript
import { axum } from "@lambda-group/axum-js";

const app = axum();
const port = 3000;

app.get('/', (req, res) => {
  res.send('Hello World!')
});

app.listen(port, () => {
  console.log(`Server is humming on port ${port}`)
});
```

## 🤝 Contributing

We're always on the lookout for contributions. Navigate to [CONTRIBUTING.md](CONTRIBUTING.md) for the scoop on how you can join the crew.

## 📜 License

Distributed under the [MIT License](LICENSE).

## 🙏 Acknowledgements

- Kudos to the Axum and Napi-rs communities for laying the groundwork that made `axum-js` feasible.
- A world of thanks to every contributor to `axum-js`, your input, no matter the magnitude, propels us forward.

---

<div align="center">
   Idealized with ❤️ by Daniel Boll
</div>
