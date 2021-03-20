import { createConnectionHandler } from "../utils/helpers";

describe("createConnectionHandler tests", () => {
  const mockedConn = {
    send: jest.fn(),
  };
  jest.useFakeTimers();

  test("[.sendBlock] should follow API contract and return correct json", () => {
    const handler = createConnectionHandler({
      socketConn: mockedConn,
      debounceTimer: 0,
    });
    const expectedBlock = JSON.stringify({
      id: "id1",
      content: {
        time: 123,
        name: "bill",
      },
    });
    handler.sendBlock("id1", { time: 123, name: "bill" });
    expect(mockedConn.send).toHaveBeenCalledTimes(1);
    expect(mockedConn.send).toHaveBeenLastCalledWith(expectedBlock);
  });

  test("[.sendMultipleBlocks] should send multiple blocks", () => {
    const handler = createConnectionHandler({
      socketConn: mockedConn,
      debounceTimer: 0,
    });
    const expectedBlock = JSON.stringify({
      id: "id1",
      content: {
        time: 123,
        name: "bill",
      },
    });
    const block = {
        blockId: "id1", 
        content: { time: 123, name: "bill" }
    }
    const blocks = [block, block, block, block]
    handler.sendMultipleBlocks(blocks);
    expect(mockedConn.send).toHaveBeenCalledTimes(4);
    expect(mockedConn.send).toHaveBeenLastCalledWith(expectedBlock);
  });
});
