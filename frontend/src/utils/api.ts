export const checkRules = async () => {
  const response = await fetch('http://localhost:3000/rules');
  return response.json();
};

export const payRansom = async (amount: number) => {
  const response = await fetch('http://localhost:3000/pay', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ amount }),
  });
  return response.json();
};

export const installMalware = async () => {
  const response = await fetch('http://localhost:3030/install', {
    method: 'POST',
  });
  return response.json();
};
