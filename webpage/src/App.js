import React, { useState, useEffect } from "react";
import RealTimeField from "./RealTimeField";

const socketConn = new WebSocket("ws://127.0.0.1:9001");

const useBlock = () => {
  let [blocks, setBlocks] = useState({});
  let blockCounter = 0;

  const updateBlock = (blockId, text) => {
    setBlocks({
      ...blocks,
      [blockId]: {
        ...blocks[blockId],
        text: text,
      },
    });
  };

  const addBlocks = (newBlocks) => {
    const newBlockStates = Array.isArray(newBlocks) ? newBlocks : [newBlocks];
    const oldState = { ...blocks };

    const newState = newBlockStates
      .map(({ content, id }) => ({
        [id]: { ...content },
      }))
      .reduce(
        (acc, newBlock) => ({
          ...acc,
          ...newBlock,
        }),
        oldState
      );

    console.log(newState);
    setBlocks(newState);
  };

  const addEmptyBlock = () => {
    let emptyId = `field_${blockCounter}`;
    while (blocks[emptyId]) {
      blockCounter += 1;
      emptyId = `field_${blockCounter}`;
    }
    const emptyBlock = {
      id: emptyId,
      content: {
        text: "",
      },
    };
    addBlocks(emptyBlock);
  };

  return {
    blocks,
    addBlocks,
    updateBlock,
    addEmptyBlock,
  };
};

const App = () => {
  let { blocks, addBlocks, updateBlock, addEmptyBlock } = useBlock();

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
      <header className="header">
        <p>WELCOME TO THE CEM WEBPAGE!</p>
      </header>

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

      <footer>
        <p>Nothing to see down here</p>
      </footer>
    </>
  );
};

export default App;
