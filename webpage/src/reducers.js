let blockCounter = 0;

const blockReducer = (state, action) => {
  switch (action.type) {
    case "update":
      const { text, blockId } = action;
      console.log(state)
      return {
        ...state,
        [blockId]: {
          ...state[blockId],
          text: text,
        },
      };

    case "add":
      const { blocks } = action;
      const newBlockStates = Array.isArray(blocks) ? blocks : [blocks];

      return newBlockStates
        .map(({ content, id }) => ({
          [id]: { ...content },
        }))
        .reduce(
          (acc, newBlock) => ({
            ...acc,
            ...newBlock,
          }),
          state
        );

    case "addEmpty":
      let emptyId = `field_${blockCounter}`;
      while (state[emptyId]) {
        blockCounter += 1;
        emptyId = `field_${blockCounter}`;
      }
      const emptyBlock = {
        id: emptyId,
        content: {
          text: "",
        },
      };
      return { ...state, [emptyId]: { ...emptyBlock } };
    case "swap":
      let {blockId1, blockId2} = action;
      console.log(action)
      let content1 = state[blockId1];
      let content2 = state[blockId2];
      state[blockId1] = content2;
      state[blockId2] = content1;
      console.log(state)
      return state
    default:
      return state
  }
};

export { blockReducer };
