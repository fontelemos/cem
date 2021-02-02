import React, { useState, useEffect } from "react";
import { updateGlobalState, updateBlock } from "./utils";
import RealTimeField from "./RealTimeField";

const socketConn = new WebSocket("ws://127.0.0.1:9001");

const App = () => {
  let id1 = "field_1";
  let [value1, setvalue1] = useState("");

  let id2 = "field_2";
  let [value2, setvalue2] = useState("");

  let id3 = "field_3";
  let [value3, setvalue3] = useState("");

  const callbackList = [
    { callbackId: id1, callback: setvalue1 },
    { callbackId: id2, callback: setvalue2 },
    { callbackId: id3, callback: setvalue3 },
  ];

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
        : updateBlock(payload, callbackList);
    };
  });

  return (
    <>
      <header className="header">
        <p>WELCOME TO THE CEM WEBPAGE!</p>
      </header>
      <RealTimeField
        socketConn={socketConn}
        fieldId={id1}
        value={value1}
        setValue={setvalue1}
      />
      <RealTimeField
        socketConn={socketConn}
        fieldId={id2}
        value={value2}
        setValue={setvalue2}
      />
      <div
        style={{ width: "100%", height: "2px", backgroundColor: "purple" }}
      ></div>
      <span> Same source inputs </span>
      <RealTimeField
        socketConn={socketConn}
        fieldId={id3}
        value={value3}
        setValue={setvalue3}
      />
      <RealTimeField
        socketConn={socketConn}
        fieldId={id3}
        value={value3}
        setValue={setvalue3}
      />
      <RealTimeField
        socketConn={socketConn}
        fieldId={id3}
        value={value3}
        setValue={setvalue3}
      />
      <footer>
        <p>Nothing to see down here</p>
      </footer>
    </>
  );
};

export default App;
