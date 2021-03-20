import React, { useCallback, useEffect, useReducer } from "react";
import AddBlockButton from "./components/AddBlockButton"
import RealTimeField from "./components/RealTimeField";
import blockReducer from "./reducers/blockReducer";
import DragBlock from "./components/DragBlock";
import PagePreview from "./PagePreview";
import { DndProvider } from "react-dnd";
import { HTML5Backend } from "react-dnd-html5-backend";
import { createConnectionHandler } from "./utils/utils";

const socketConn = new WebSocket("ws://127.0.0.1:9001");
const debounceTimer = 300;

const BlockForm = () => {
  let [blocks, dispatch] = useReducer(blockReducer, {});
  let { sendBlock, sendMultipleBlocks } = createConnectionHandler({
    socketConn,
    debounceTimer,
  });

  let connectedSendBlock = useCallback(
    (blockId, content) => {
      dispatch({ blockId, content, type: "update" });
      sendBlock(blockId, content);
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
    <section className="page">
      <div className="page__admin">
        <AddBlockButton dispatch={dispatch} />
        {Object.keys(blocks).map((fieldName) => (
          <RealTimeField
            {...blocks[fieldName]}
            key={fieldName}
            blockId={fieldName}
            sendBlock={connectedSendBlock}
          />
        ))}

        <DndProvider backend={HTML5Backend}>
          {Object.keys(blocks).map((fieldName) => (
            <DragBlock
              key={`${fieldName}--drag`}
              blockId={fieldName}
              text={blocks[fieldName].text}
              swapBlock={connectedSwapBlock}
            />
          ))}
        </DndProvider>
      </div>

      <PagePreview blocks={blocks} />
    </section>
  );
};

export default BlockForm;
