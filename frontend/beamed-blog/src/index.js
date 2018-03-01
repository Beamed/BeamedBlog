import React from 'react';
import ReactDOM from 'react-dom';
import './ext_dependencies/bootswatch_cyborg.css';
import './styles/index.css';
import App from './App.js';

import AppHeader from './components/AppHeader';
import {BrowserRouter} from 'react-router-dom';
import {Route} from 'react-router-dom';
import Home from './pages/Home';
import Login from './pages/Login';
import registerServiceWorker from './registerServiceWorker';

ReactDOM.render(     
    <div className="app">
        <BrowserRouter>
            <App/>
        </BrowserRouter>
    </div>
, document.getElementById('root'));
registerServiceWorker();
