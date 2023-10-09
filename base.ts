import { AxumApp } from "./index";
import { performance } from "node:perf_hooks";

const startTime = performance.now();

const app = new AxumApp();

app.get("/", (_, response) => {
  response.status(200).sendText("Hello from AxumJS!");
});

app.post("/user", (request, response) => {
  const body = request.body;
  console.log("Payload from Rust:", body);

  response
    .status(201)
    .sendJson({ message: "Hello from AxumJS!", payload: body });
});

app.listen(3000, () => {
  const endTime = performance.now();
  const duration = endTime - startTime;

  console.log(`🚀 Server started in ${duration.toFixed(2)}ms`);
  console.log("🚀 Server listening on port 3000");
});
