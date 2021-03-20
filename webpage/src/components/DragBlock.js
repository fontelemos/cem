import React from "react";
import { useDrag, useDrop } from "react-dnd";
import { ITEM_TYPES } from "../utils/constants";

const DragBlock = ({ isDragging, text, swapBlock, blockId }) => {
  const [{ opacity }, dragRef] = useDrag(
    () => ({
      item: { type: ITEM_TYPES.DragBlock, text, blockId: blockId },
      collect: (monitor) => ({
        opacity: monitor.isDragging() ? 0.5 : 1,
      }),
    }),
    [text, blockId]
  );

  const [, drop] = useDrop(() => ({
    accept: ITEM_TYPES.DragBlock,
    drop: (droppedProps) => swapBlock(droppedProps.blockId, blockId),
  }));

  return (
    <div ref={drop}>
      <div ref={dragRef} style={{ opacity }}>
        {text}
      </div>
    </div>
  );
};

export default DragBlock;
