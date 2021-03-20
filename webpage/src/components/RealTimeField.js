import React, { useCallback } from "react";

const RealTimeField = React.memo(({ blockId, text, sendBlock }) => {
  const handleOnChange = useCallback(
    (event) => {
      let content = {
        text: event.target.value
      }
      sendBlock(blockId, content);
    },
    [blockId, sendBlock]
  );

  return (
    <div>
      <label>Block: {blockId} </label>
      <input type="text" value={text || ""} onChange={handleOnChange}></input>
    </div>
  );
});

export default RealTimeField;
