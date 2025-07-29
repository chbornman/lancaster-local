import { useEffect } from 'react'
import { Routes, Route } from 'react-router-dom'
import { useTranslation } from 'react-i18next'
import { Helmet } from 'react-helmet-async'
import Layout from './components/Layout'
import NewsFeed from './pages/NewsFeed'
import Calendar from './pages/Calendar'
import SubmitPost from './pages/SubmitPost'
import SubmitEvent from './pages/SubmitEvent'
import AdminLogin from './pages/AdminLogin'
import AdminDashboard from './pages/AdminDashboard'
import { useDirection } from './hooks/useDirection'

function App(): JSX.Element {
  const { i18n } = useTranslation()
  const { setDirection } = useDirection()

  useEffect(() => {
    const rtlLanguages = import.meta.env.VITE_RTL_LANGUAGES?.split(',') || ['ar', 'he', 'fa', 'ur']
    const isRTL = rtlLanguages.includes(i18n.language)
    setDirection(isRTL ? 'rtl' : 'ltr')
    document.documentElement.dir = isRTL ? 'rtl' : 'ltr'
    document.documentElement.lang = i18n.language
  }, [i18n.language, setDirection])

  return (
    <>
      <Helmet>
        <title>Lancaster Local</title>
      </Helmet>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<NewsFeed />} />
          <Route path="calendar" element={<Calendar />} />
          <Route path="submit/post" element={<SubmitPost />} />
          <Route path="submit/event" element={<SubmitEvent />} />
          <Route path="admin" element={<AdminLogin />} />
          <Route path="admin/dashboard" element={<AdminDashboard />} />
        </Route>
      </Routes>
    </>
  )
}

export default App