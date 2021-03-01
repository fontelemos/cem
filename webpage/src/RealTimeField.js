import React, { useCallback } from "react";
import debounce from "lodash.debounce";

const RealTimeField = React.memo(
  ({ socketConn, blockId, text, blockDispatch }) => {
    const DEBOUNCE_TIMER = 100;

    const sendText = (text) => {
      let payload = {
        id: blockId,
        content: {
          time: `${Date.now()}`,
          text: `${text}`,
        },
      };
      console.log("Sending new block to friends!!!");
      socketConn.send(JSON.stringify(payload));
    };

    //eslint-disable-next-line
    const debouncedSendText = useCallback(
      debounce((text) => sendText(text), DEBOUNCE_TIMER),
      []
    );

    const handleOnChange = useCallback(
      (event) => {
        let text = event.target.value;
        blockDispatch({ blockId, text, type: "update" });
        debouncedSendText(text);
      },
      [blockId, blockDispatch, debouncedSendText]
    );

    return (
      <div>
        <label>Block: {blockId} </label>
        <input type="text" value={text || ""} onChange={handleOnChange}></input>
      </div>
    );
  }
);

export default RealTimeField;
