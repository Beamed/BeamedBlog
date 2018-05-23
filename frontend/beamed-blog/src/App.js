import React, { Component } from 'react';
import './styles/App.css';
import AppHeader from './components/AppHeader';
import {Router, Route} from 'react-router';
import Sidebar from './components/Sidebar';
import Content from './components/Content';
import Footer from './components/Footer';
import Main from './Main.js';


class App extends Component {
  render() {
    return (
      <div className="app">
        <AppHeader />
        <Main />
        <Footer />
      </div>
    );
  }
}

export default App;
