const blockReducer = (state, action) => {
  switch (action.type) {
    case "update": {
      let { content, blockId } = action;
      const oldBlock = state[blockId];
      return oldBlock
        ? {
            ...state,
            [blockId]: {
              ...oldBlock,
              ...content
            },
          }
        : state;
    }

    case "add": {
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
    }

    case "addEmpty": {
      let { emptyKeys, blockId } = action;
      if (!blockId) {
        console.warn("[addEmpty] Unable to add block, ID already in use");
        return state;
      }

      emptyKeys = emptyKeys ?? { text: "" };
      const emptyBlock = {
        [blockId]: {
          ...emptyKeys,
        },
      };
      return { ...state, ...emptyBlock };
    }

    case "swap": {
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
    }
    default:
      return state;
  }
};

export default blockReducer;
