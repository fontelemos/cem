import React, { useCallback } from "react";

const RealTimeField = React.memo(({ blockId, text, sendBlock }) => {
  const handleOnChange = useCallback(
    (event) => {
      let text = event.target.value;
      sendBlock(blockId, text);
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
