import React from 'react';
import { useRouter } from 'next/router';

const Success: React.FC = () => {
  const router = useRouter();

  React.useEffect(() => {
    const timer = setTimeout(() => {
      router.push('/');
    }, 5000);

    return () => clearTimeout(timer);
  }, [router]);

  return (
    <div>
      <h1>Decryption Successful!</h1>
      <p>You will be redirected to the home page in 5 seconds.</p>
    </div>
  );
};

export default Success;
