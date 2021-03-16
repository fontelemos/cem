import React from "react";

const PagePreview = ({ blocks }) => {
  return (
    <aside className={"page__preview"}>
      <header>
        <h1>Super webpage</h1>
      </header>

      <div className={"page__preview__content"}>
        {Object.keys(blocks).map((fieldName) => (
          <div key={fieldName} className={"page__preview__content__square"}>
            <p> {blocks[fieldName]?.text}</p>
          </div>
        ))}
      </div>

      <footer>
        <p>Footer</p>
      </footer>
    </aside>
  );
};

export default PagePreview;
