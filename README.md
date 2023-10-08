<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://socialify.git.ci/daniel-boll/axum-js/image?description=1&descriptionEditable=%F0%9F%9A%80%20A%20Axum%20http%20wrapper%20for%20NodeJS%20built%20in%20Rust%20%F0%9F%A6%80&font=Inter&issues=1&language=1&name=1&owner=1&pattern=Solid&stargazers=1&theme=Dark">
    <img src="https://socialify.git.ci/daniel-boll/axum-js/image?description=1&descriptionEditable=%F0%9F%9A%80%20A%20Axum%20http%20wrapper%20for%20NodeJS%20built%20in%20Rust%20%F0%9F%A6%80&font=Inter&issues=1&language=1&name=1&owner=1&pattern=Solid&stargazers=1&theme=Dark" alt="axum-js" />
  </picture>
  <br/>
  <br/>
  
![Tests Status][badge-tests]
![Deploy Status][badge-deploy]

</div>

[badge-tests]:https://img.shields.io/github/actions/workflow/status/daniel-boll/axum-js/test.yml?branch=main&label=tests&logo=github&style=for-the-badge
[badge-deploy]:https://img.shields.io/github/actions/workflow/status/daniel-boll/axum-js/publish.yml?branch=main&label=deploy&logo=github&style=for-the-badge


<div align="center">

[![NPM version](https://img.shields.io/npm/v/@lambda-group/axum-js.svg?style=flat-square)](https://www.npmjs.com/package/@lambda-group/axum-js)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)](CONTRIBUTING.md)
[![Twitter Follow](https://img.shields.io/twitter/follow/db_regret?style=social)](https://x.com/db_regret)

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
