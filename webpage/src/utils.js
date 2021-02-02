const updateBlock = (payload, callbackList) => {
  const id = payload?.id;
  const newText = payload?.content?.text;
  const callback = callbackList.find((element) => element.callbackId === id)?.callback;
  if (callback) {
    console.log("Received new block from friend!!");
    callback(newText);
  }
};

const updateGlobalState = (payloadArray, callbackList) => {
  callbackList.forEach(({ callbackId, callback }) => {
    payloadArray.forEach(({ id, content }) => {
      if (id === callbackId) {
        callback(content?.text);
        console.log("updated:", id);
      }
    });
  });
};

export { updateBlock, updateGlobalState };
