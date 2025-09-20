import React from 'react';

interface PhishingButtonProps {
  onClick: (choice: 'Yes' | 'No') => void;
}

const PhishingButton: React.FC<PhishingButtonProps> = ({ onClick }) => {
  return (
    <div>
      <button onClick={() => onClick('Yes')}>Delete All Files (Yes)</button>
      <button onClick={() => onClick('No')}>Delete All Files (No)</button>
    </div>
  );
};

export default PhishingButton;
