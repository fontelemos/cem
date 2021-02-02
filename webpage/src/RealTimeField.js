import React, { useCallback } from "react";
import debounce from "lodash.debounce";

const RealTimeField = ({ socketConn, fieldId, value, setValue }) => {
    const DEBOUNCE_TIMER = 100;
  
    const sendText = (text) => {
      let payload = {
        id: fieldId,
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
  
    const handleOnChange = (event) => {
      let text = event.target.value;
      setValue(text);
      debouncedSendText(text);
    };
  
    return (
      <div>
        <input type="text" value={value} onChange={handleOnChange}></input>
      </div>
    );
  };
  
  export default RealTimeField;
  