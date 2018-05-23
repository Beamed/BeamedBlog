import React, { Component } from 'react';
import {Navbar, Nav, NavItem} from 'react-bootstrap';
import { Link } from 'react-router-dom';
import '../styles/AppHeader.css';
import AuthNavItem from '../components/AuthNavItem';

class AppHeader extends Component {
    constructor(props) {
        super(props);
        this.state = {};
    }

    render() {
        return (
            <Navbar inverse collapseOnSelect className="col-md-8 col-md-offset-2">
                <Navbar.Header>
                    <Navbar.Brand>
                        Beamed Blog
                    </Navbar.Brand>
                    <Navbar.Toggle/>
                </Navbar.Header>
                <Navbar.Collapse>
                    <Nav>
                        <NavItem eventKey={1} href='/'>
                            Recent Posts
                        </NavItem>
                        <NavItem eventKey={2} href="/archive">
                            Archive
                        </NavItem>
                    </Nav>
                    <Nav pullRight>
                        <NavItem  eventKey={3} href='/about'>
                            About Me
                        </NavItem>
                        <AuthNavItem eventKey={4} />
                    </Nav>
                </Navbar.Collapse>

            </Navbar>
        );
    }
}

export default AppHeader;