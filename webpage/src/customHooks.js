import { useState } from "react";

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

  return [
    blocks,
    {
      addBlocks,
      updateBlock,
      addEmptyBlock,
    },
  ];
};

export { useBlock };
