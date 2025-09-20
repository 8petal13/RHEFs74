import React from 'react';

const Instructions: React.FC = () => {
  return (
    <div>
      <h1>Your Files Have Been Encrypted</h1>
      <p>To decrypt your files, follow these rules:</p>
      <ol>
        <li>Do not contact the police, media, or any other authorities.</li>
        <li>You have 24 hours to pay the ransom of $20,000.</li>
        <li>Failure to pay the complete amount will result in the autodelete of your files.</li>
      </ol>
    </div>
  );
};

export default Instructions;
