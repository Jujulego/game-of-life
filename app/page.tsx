import Universe from '@/app/Universe';

export default function Home() {
  return (
    <main>
      { typeof document !== 'undefined' && <Universe /> }
    </main>
  )
}
