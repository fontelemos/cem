import { blockReducer } from "../reducers";

describe("blockReducer tests", () => {
  const MOCK_STATE = {
    mock1: {
      a: 2,
      b: "booo",
    },
    mock2: {
      value: 345,
    },
  };

  const MOCK_BLOCK_1 = {
    content: {
      text: "heloooo",
      randomAttribute: 12
    },
    id: "myKey",
  };

  const MOCK_BLOCK_2 = {
    content: {
      text: "GOoD MorNing"
    },
    id: "secondkey",
  };

  test("[addEmpty] should add block to the end of the state", () => {
    const expectedState = {
      ...MOCK_STATE,
      field_0: {
        text: "",
      },
    };
    let action = { type: "addEmpty", blockId: "field_0" };
    let resultState = blockReducer(MOCK_STATE, action);
    expect(resultState).toEqual(expectedState);
  });

  test("[addEmpty] should add keys", () => {
    const emptyKeys = {
      text: "",
      randomStuff: 12,
      order: 1232
    }
    const expectedState = {
      ...MOCK_STATE,
      customKey: {
        ...emptyKeys
      },
    };
    let action = { type: "addEmpty", blockId: "customKey",  emptyKeys };
    let tempState = blockReducer(MOCK_STATE, action);
    let resultState = blockReducer(tempState, action);
    expect(resultState).toEqual(expectedState);
  });

  test("[add] should add block to state", () => {

    let action = {
      blocks: MOCK_BLOCK_1,
      type: "add",
    };
    const expectedState = {
      ...MOCK_STATE,
      [MOCK_BLOCK_1.id]: {
        ...MOCK_BLOCK_1.content,
      },
    };

    let resultState = blockReducer(MOCK_STATE, action);
    expect(resultState).toEqual(expectedState);
  });

  test("[add] should add array of blocks to state (used for initial form update)", () => {
    const blocks = [MOCK_BLOCK_1, MOCK_BLOCK_2]
    let action = {
      blocks,
      type: "add",
    };
    const expectedState = {
      ...MOCK_STATE,
      [MOCK_BLOCK_1.id]: {
        ...MOCK_BLOCK_1.content,
      },
      [MOCK_BLOCK_2.id]: {
        ...MOCK_BLOCK_2.content,
      }
    };
    let resultState = blockReducer(MOCK_STATE, action);
    expect(resultState).toEqual(expectedState);
  });

  test("[update] should only change target block with same blockId", () => {
    let action = {
      content: {
        text: "surprise! new text here!"
      },
      blockId: "very secret id",
      type: "update",
    };
    const custom_state = {
      ...MOCK_STATE,
      "very secret id": {
        text: "nothing here"
      }
    }
    const expectedState = {
      ...MOCK_STATE,
      "very secret id": {
        text: "surprise! new text here!"
      }
    };
    let resultState = blockReducer(custom_state, action);
    expect(resultState).toEqual(expectedState);
  });

  test("[update] should do nothing if blockId doesn't exist", () => {
    let action = {
      text: "surprise! new text here!",
      blockId: "i don't exist!!!!!",
      type: "update",
    };
    const custom_state = {
      ...MOCK_STATE,
      "very secret id": {
        text: "nothing here"
      }
    }
    let resultState = blockReducer(custom_state, action);
    expect(resultState).toEqual(custom_state);
  });
});
