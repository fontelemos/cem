const updateBlock = (payload, blocks, buildNewBlock) => {
  const id = payload?.id;
  const newText = payload?.content?.text;
  const storedContent = blocks[id];

  if (storedContent) {
    console.log("Updated old block!");
    return {
      [id]: {
        ...storedContent,
        value: newText,
      },
    };
  }
  console.log("Received new block from friend!!");
  return buildNewBlock(id, newText);
};

const updateBlockList = (payloadArray, blocks, buildNewBlock) => {
  let newBlockstate = {};
  payloadArray.forEach((payload) => {
    let newBlock = updateBlock(payload, blocks, buildNewBlock);
    newBlockstate = {
      ...newBlockstate,
      ...newBlock,
    };
  });
  return newBlockstate;
};

export { updateBlock, updateBlockList };
