import { Switch, Route } from 'react-router-dom'
import React from 'react';
import Home from './pages/Home';
import Login from './pages/Login';

const Main = () => (
  <main>
    <Switch>
      <Route exact path='/' component={Home}/>
      <Route path='/login' component={Login}/>
    </Switch>
  </main>
)

export default Main;