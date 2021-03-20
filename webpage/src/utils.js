import debounce from "lodash.debounce";

const createConnectionHandler = ({ socketConn, debounceTimer }) => {
  const buildBlock = (blockId, content) => {
    return {
      id: blockId,
      content: {
        time: `${Date.now()}`,
        ...content
      },
    };
  };

  const sendBlock = (blockId, text) => {
    let payload = buildBlock(blockId, text);
    console.log("Sending new block to friends!!!");
    console.log(payload);
    socketConn.send(JSON.stringify(payload));
  };

  const sendMultipleBlocks = (blocks) => {
    blocks.forEach(({ blockId, text }) => {
      console.log(`sending block:${blockId} with text:${text}`);
      socketConn.send(JSON.stringify(buildBlock(blockId, text)));
    });
  };

  const debouncedSendBlock = debounce(
    (blockId, text) => sendBlock(blockId, text),
    debounceTimer
  );
  return {
    sendBlock: debouncedSendBlock,
    sendMultipleBlocks,
  };
};

export { createConnectionHandler };
