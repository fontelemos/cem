import React, { useEffect, useReducer } from "react";
import RealTimeField from "./RealTimeField";
import { blockReducer } from "./reducers";

const socketConn = new WebSocket("ws://127.0.0.1:9001");

const BlockForm = () => {
  let [blocks, dispatch] = useReducer(blockReducer, {});

  useEffect(() => {
    socketConn.onopen = () => {
      console.log("WebSocket Client Connected");
    };

    socketConn.onmessage = (message) => {
      console.log("======MESSAGE RECEIVED======");
      console.log(message.data);
      const payload = JSON.parse(message.data);
      dispatch({ blocks: payload, type: "add" });
    };
  });

  return (
    <>
      <button onClick={() => dispatch({ type: "addEmpty" })}> Add blocks! </button>

      <section>
        {Object.keys(blocks).map((fieldName) => (
          <RealTimeField
            {...blocks[fieldName]}
            key={fieldName}
            blockId={fieldName}
            blockDispatch={dispatch}
            socketConn={socketConn}
          />
        ))}
      </section>
    </>
  );
};

export default BlockForm;
