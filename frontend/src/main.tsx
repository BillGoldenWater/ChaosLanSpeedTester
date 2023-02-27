import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom/client";
import "normalize.css";
import { Button, Center, ChakraProvider, VStack } from "@chakra-ui/react";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ChakraProvider>
      <App />
    </ChakraProvider>
  </React.StrictMode>
);

function App() {
  const [testing, setTesting] = useState(false);
  const [progress, setProgress] = useState(0);
  const [speed, setSpeed] = useState(0);

  useEffect(() => {
    if (testing) {
      let done = false;
      let first = true;
      let lastProgress: ProgressEvent | null = null;
      let lastTime = Date.now();
      let bpsAverage = 0;

      let xhr = new XMLHttpRequest();
      xhr.responseType = "arraybuffer";
      xhr.addEventListener("progress", (e) => {
        let size: number; // byte
        let time: number; // millis

        if (lastProgress != null) {
          size = e.loaded - lastProgress.loaded;
          time = Date.now() - lastTime;
        } else {
          size = e.loaded;
          time = e.timeStamp;
        }

        let bps = size * (1000 / time);

        if (first) {
          bpsAverage = bps;
          first = false;
        }

        bpsAverage += (bps - bpsAverage) / 8;
        setProgress(e.loaded / e.total);
        setSpeed(bpsAverage);

        lastProgress = e;
        lastTime = Date.now();
      });
      xhr.addEventListener("load", () => {
        setTesting(false);
        done = true;
      });
      // noinspection HttpUrlsUsage
      xhr.open(
        "GET",
        `http://${window.location.hostname}:25545/gen/1000000000`
      );
      xhr.send();

      return () => (done ? undefined : xhr.abort());
    }
  }, [testing]);

  return (
    <Center minH={"100vh"}>
      <VStack>
        <Button onClick={setTesting.bind(null, !testing)}>
          {testing ? "Abort" : "Run"}
        </Button>
        <div>{(progress * 100).toFixed(3)}%</div>
        <div>{((speed / 1000 / 1000) * 8).toFixed(3)} Mbps</div>
      </VStack>
    </Center>
  );
}
