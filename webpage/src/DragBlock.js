import React from "react";
import { useDrag, useDrop } from "react-dnd";
import { ITEM_TYPES } from "./constants";

/**
 * Your Component
 */
const DragBlock = ({ isDragging, text, blockDispatch, blockId }) => {
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
    drop: (droppedProps) => {
      blockDispatch({
        blockId1: droppedProps.blockId,
        blockId2: blockId,
        type: "swap",
      });
    },
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
