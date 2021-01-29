import React, { useState, useCallback, useEffect } from "react";
import debounce from "lodash.debounce";

const socketConn = new WebSocket("ws://127.0.0.1:9001");

const App = () => {
  let [value, setvalue] = useState("");

  useEffect(() => {
    socketConn.onopen = () => {
      console.log("WebSocket Client Connected");
    };
    socketConn.onmessage = (message) => {
      console.log("======MESSAGE RECEIVED======");
      console.log(message.data);
    };
  }, []);

  const sendText = (text) => {
    let payload = {
      id: "129837ab890qlc8",
      content: {
        time: `${Date.now()}`,
        text: `${text}`,
      },
    };
    console.log("====== Sending message ======");
    socketConn.send(JSON.stringify(payload));
  };

  const debouncedSendText = useCallback(
    debounce((text) => sendText(text), 300),
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