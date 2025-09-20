import React, { useState } from 'react';

interface KeyFormProps {
  onKeySubmit: (key: string) => void;
}

const KeyForm: React.FC<KeyFormProps> = ({ onKeySubmit }) => {
  const [key, setKey] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onKeySubmit(key);
  };

  return (
    <form onSubmit={handleSubmit}>
      <label>
        Enter Key:
        <input
          type="text"
          value={key}
          onChange={(e) => setKey(e.target.value)}
        />
      </label>
      <button type="submit">Submit Key</button>
    </form>
  );
};

export default KeyForm;
