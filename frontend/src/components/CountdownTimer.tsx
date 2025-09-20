import React from 'react';

interface CountdownTimerProps {
  timeLeft: number;
}

const CountdownTimer: React.FC<CountdownTimerProps> = ({ timeLeft }) => {
  const hours = Math.floor(timeLeft / 3600);
  const minutes = Math.floor((timeLeft % 3600) / 60);
  const seconds = timeLeft % 60;

  return (
    <div>
      <h2>Time Left to Pay: {hours}:{minutes}:{seconds}</h2>
    </div>
  );
};

export default CountdownTimer;
