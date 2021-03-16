import React, { useCallback, useEffect, useReducer } from "react";
import RealTimeField from "./RealTimeField";
import { blockReducer } from "./reducers";
import DragBlock from "./DragBlock";
import { DndProvider } from "react-dnd";
import { HTML5Backend } from "react-dnd-html5-backend";
import { createConnectionHandler } from "./utils";

const socketConn = new WebSocket("ws://127.0.0.1:9001");
const debounceTimer = 300;

const BlockForm = () => {
  let [blocks, dispatch] = useReducer(blockReducer, {});
  let { sendBlock, sendMultipleBlocks } = createConnectionHandler({
    socketConn,
    debounceTimer,
  });

  let connectedSendBlock = useCallback(
    (blockId, text) => {
      dispatch({ blockId, text, type: "update" });
      sendBlock(blockId, text);
    },
    [sendBlock]
  );

  let connectedSwapBlock = useCallback(
    (blockId1, blockId2) => {
      dispatch({
        blockId1,
        blockId2,
        type: "swap",
        callback: sendMultipleBlocks,
      });
    },
    [sendMultipleBlocks]
  );

  useEffect(() => {
    socketConn.onopen = () => {
      console.log("WebSocket Client Connected");
    };

    socketConn.onmessage = (message) => {
      console.log("======MESSAGE RECEIVED======");
      console.log(message.data);
      const payload = JSON.parse(message.data);
      dispatch({ blocks: payload, type: "add" });
    };
  });

  return (
    <DndProvider backend={HTML5Backend}>
      <button onClick={() => dispatch({ type: "addEmpty" })}>
        Add blocks!
      </button>

      <section>
        {Object.keys(blocks).map((fieldName) => (
          <RealTimeField
            {...blocks[fieldName]}
            key={fieldName}
            blockId={fieldName}
            sendBlock={connectedSendBlock}
          />
        ))}
      </section>

      <section>
        {Object.keys(blocks).map((fieldName) => (
          <DragBlock
            key={`${fieldName}--drag`}
            blockId={fieldName}
            text={blocks[fieldName].text}
            swapBlock={connectedSwapBlock}
          />
        ))}
      </section>
    </DndProvider>
  );
};

export default BlockForm;
