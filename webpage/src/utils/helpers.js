import debounce from "lodash.debounce";

const createConnectionHandler = ({ socketConn, debounceTimer = 100 }) => {
  const buildBlock = (blockId, content) => {
    return {
      id: blockId,
      content: {
        time: `${Date.now()}`,
        ...content
      },
    };
  };

  const sendBlock = (blockId, content) => {
    let payload = buildBlock(blockId, content);
    let jsonContent = JSON.stringify(payload)
    console.log("Sending new block to friends!!!");
    console.log(payload);
    socketConn.send(jsonContent);
  };

  const sendMultipleBlocks = (blocks) => {
    blocks.forEach(({ blockId, content }) => {
      console.log(`sending block:${blockId} with content:${content}`);
      socketConn.send(JSON.stringify(buildBlock(blockId, content)));
    });
  };

  const debouncedSendBlock = debounce(
    (blockId, text) => sendBlock(blockId, text),
    debounceTimer
  );
  return {
    sendBlock: debounceTimer? debouncedSendBlock: sendBlock,
    sendMultipleBlocks,
  };
};

export { createConnectionHandler };
