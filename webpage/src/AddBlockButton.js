import React, { useCallback, useMemo, useState } from "react"

const useInput = (defaultValue) => {
  let [text, setText ] = useState(defaultValue)

  const handleOnChange = (event) => {
    setText(event.target.value)
  }
  return [text, handleOnChange]
}

const AddBlockButton = ({ dispatch }) => {

  let [text, handleOnChange ] = useInput("")

  const dispatchBlock = useCallback(() => {
    dispatch({
      type: "addEmpty",
      blockId: text
    })
  }, [text, dispatch])

  return (
    <div>
      <label>block id: </label>
      <input type="text" value={text} onChange={handleOnChange}></input>
      <button onClick={dispatchBlock}>
        Add block!
      </button>
    </div>
  );
};


export default AddBlockButton