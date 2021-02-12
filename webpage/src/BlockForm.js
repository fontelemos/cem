import React, { useEffect } from "react";
import { useBlock } from "./customHooks.js";
import RealTimeField from "./RealTimeField";

const socketConn = new WebSocket("ws://127.0.0.1:9001");

const BlockForm = () => {
  let [blocks, { addBlocks, updateBlock, addEmptyBlock }] = useBlock();

  useEffect(() => {
    socketConn.onopen = () => {
      console.log("WebSocket Client Connected");
    };

    socketConn.onmessage = (message) => {
      console.log("======MESSAGE RECEIVED======");
      console.log(message.data);
      const payload = JSON.parse(message.data);
      addBlocks(payload);
    };
  });

  return (
    <>
      <button onClick={addEmptyBlock}> Add blocks! </button>

      <section>
        {Object.keys(blocks).map((fieldName) => (
          <RealTimeField
            {...blocks[fieldName]}
            key={fieldName}
            blockId={fieldName}
            setBlockText={updateBlock}
            socketConn={socketConn}
          />
        ))}
      </section>
    </>
  );
};

export default BlockForm;
