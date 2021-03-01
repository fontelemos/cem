let blockCounter = 0;

const blockReducer = (state, action) => {
  switch (action.type) {
    case "update":
      const { text, blockId } = action;
      console.log(state);
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
        [emptyId]: {
          text: "",
        },
      };

      return { ...state, ...emptyBlock };
    case "swap":
      let { blockId1, blockId2, callback } = action;
      const newState = { ...state };
      newState[blockId1] = state[blockId2];
      newState[blockId2] = state[blockId1];
      callback([
        {
          blockId: blockId1,
          text: newState[blockId1].text,
        },
        {
          blockId: blockId2,
          text: newState[blockId2].text,
        },
      ]); // TODO REFACTOR this weird interface!
      return newState;
    default:
      return state;
  }
};

export { blockReducer };
