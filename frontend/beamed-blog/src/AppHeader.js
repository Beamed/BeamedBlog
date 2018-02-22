import React, { Component } from 'react';
import {Navbar, Nav, NavItem} from 'react-bootstrap';
import './AppHeader.css';

class AppHeader extends Component {
    render() {
        return (
            <Navbar inverse collapseOnSelect>
                <Navbar.Header>
                    <Navbar.Brand>
                        <a href="#home">Beamed Blog</a>
                    </Navbar.Brand>
                </Navbar.Header>
                <Navbar.Collapse>
                    <Nav>
                        <NavItem eventKey={1} href="#">
                            Recent Posts
                        </NavItem>
                        <NavItem eventKey={2} href="#">
                            Archive
                        </NavItem>
                    </Nav>
                    <Nav pullRight>
                        <NavItem  eventKey={3} href="#">
                            About Me
                        </NavItem>
                    </Nav>
                </Navbar.Collapse>

            </Navbar>
        );
    }
}

export default AppHeader;