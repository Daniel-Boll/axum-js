import { AxumApp } from "./index";

const app = new AxumApp();

app.get("/", (request, response) => {
  const inJs = request.body();
  console.log("Payload from Rust:", inJs);

  // response.status(201).sendJson({ message: "Hello from AxumJS!" });
  response.status(201).sendText("Hello from AxumJS!");
});

app.listen(3000);
