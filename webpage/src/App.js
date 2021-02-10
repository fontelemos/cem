import React, { useState, useEffect } from "react";
import { updateBlockList, updateBlock } from "./utils";
import RealTimeField from "./RealTimeField";

const socketConn = new WebSocket("ws://127.0.0.1:9001");

const App = () => {
  let [blocks, setblocks] = useState({});
  let [localBlockCounter, setLocalBlockCounter] = useState(0);

  useEffect(() => {
    socketConn.onopen = () => {
      console.log("WebSocket Client Connected");
    };

    socketConn.onmessage = (message) => {
      console.log("======MESSAGE RECEIVED======");
      console.log(message.data);
      const payload = JSON.parse(message.data);
      const newBlockState = Array.isArray(payload)
        ? updateBlockList(payload, blocks, buildNewBlock)
        : updateBlock(payload, blocks, buildNewBlock);

      setblocks({ ...blocks, ...newBlockState });
    };
  });

  const buildNewBlock = (blockId, textValue) => {
    return {
      [blockId]: {
        key: blockId,
        blockId: blockId,
        value: textValue,
        socketConn: socketConn,
      },
    };
  };

  const appendEmptyBlock = () => {
    let tempCounter = localBlockCounter;
    while (blocks[`field_${tempCounter}`]) {
      tempCounter++;
    }
    setLocalBlockCounter(tempCounter);
    const blockId = `field_${tempCounter}`;
    const newBlockContent = buildNewBlock(blockId, "");

    setblocks({
      ...blocks,
      ...newBlockContent,
    });
  };

  const buildSetValue = (fieldName) => (text) => {
    setblocks({
      ...blocks,
      [fieldName]: {
        ...blocks[fieldName],
        value: text,
      },
    });
  };

  const blockElements = [];
  for (let fieldName in blocks) {
    const setValue = buildSetValue(fieldName);
    blockElements.push(
      <RealTimeField {...blocks[fieldName]} setValue={setValue} />
    );
  }

  return (
    <>
      <header className="header">
        <p>WELCOME TO THE CEM WEBPAGE!</p>
      </header>
      <button onClick={appendEmptyBlock}> Add blocks! </button>

      <section>{blockElements}</section>

      <footer>
        <p>Nothing to see down here</p>
      </footer>
    </>
  );
};

export default App;
