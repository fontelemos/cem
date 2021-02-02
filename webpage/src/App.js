import React, { useState, useCallback, useEffect } from "react";
import debounce from "lodash.debounce";

const socketConn = new WebSocket("ws://127.0.0.1:9001");

const updateBlock = (payload, callback) => {
  const newText = payload?.content?.text;
  if (newText) {
    console.log("Updating block!");
    callback(newText);
  }
};

const updateGlobalState = (payloadArray, callbackList) => {
  callbackList.forEach(({ callbackId, callback }) => {
    payloadArray.forEach(({ id, content }) => {
      if (id === callbackId) {
        callback(content?.text)
        console.log("updated:", id);
      };
    });
  });
};

const App = () => {
  const DEBOUNCE_TIMER = 300;
  const BLOCK_ID = "129837ab890qlc8";

  let [value, setvalue] = useState("");

  const callbackList = [{ callbackId: BLOCK_ID, callback: setvalue }]

  useEffect(() => {
    socketConn.onopen = () => {
      console.log("WebSocket Client Connected");
    };
    socketConn.onmessage = (message) => {
      console.log("======MESSAGE RECEIVED======");
      console.log(message.data);
      const payload = JSON.parse(message.data);
      Array.isArray(payload)
        ? updateGlobalState(payload, callbackList)
        : updateBlock(payload, setvalue);
    };
  }, []);

  const sendText = (text) => {
    let payload = {
      id: BLOCK_ID,
      content: {
        time: `${Date.now()}`,
        text: `${text}`,
      },
    };
    console.log("====== Sending message ======");
    socketConn.send(JSON.stringify(payload));
  };

  //eslint-disable-next-line
  const debouncedSendText = useCallback(
    debounce((text) => sendText(text), DEBOUNCE_TIMER),
    []
  );

  const handleOnChange = (event) => {
    let text = event.target.value;
    setvalue(text);
    debouncedSendText(text);
  };

  return (
    <>
      <header className="header">
        <p>WELCOME TO THE CEM WEBPAGE!</p>
      </header>
      <input type="text" value={value} onChange={handleOnChange}></input>
      <footer>
        <p>Nothing to see down here</p>
      </footer>
    </>
  );
};

export default App;
