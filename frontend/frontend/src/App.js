import React, {Component} from "react";
import Cookies from 'universal-cookie';
import {
    Route,
    NavLink,
    HashRouter
} from "react-router-dom";

import Blog from "./Blog";
import NewPost from "./NewPost";
import Contact from "./Contact";
import Login from "./Login";
import Logout from "./Logout";

const cookies = new Cookies();

class App extends Component {
    renderLoginButton() {
        let token = cookies.get('token');
        if (token != null) {
            return (
                <NavLink to="/logout">Wyloguj</NavLink>
            )
        }

        return (
            <NavLink to="/login">Zaloguj</NavLink>
        )
    }

    renderNewPostButton() {
        let token = cookies.get('token');
        let rights = cookies.get('rights');
        if (token != null && rights != null && rights === 'ADMIN') {
            return (
                <li><NavLink to="/newpost">Dodaj post</NavLink></li>
            )
        }

        return (
            <li/>
        )
    }

    render() {
        return (
            <HashRouter>
                <div>
                    <h1>Blog</h1>
                    <ul className="header">
                        <li><NavLink exact to="/">Wpisy</NavLink></li>
                        <li><NavLink to="/contact">Kontakt</NavLink></li>
                        {/*<li><NavLink to="/newpost">Dodaj post</NavLink></li>*/}
                        {this.renderNewPostButton()}
                        {/*<li><NavLink to="/settings">Ustawienia</NavLink></li>*/}
                        {/*<li><NavLink to="/login">Logowanie</NavLink></li>*/}
                        <li>{this.renderLoginButton()}</li>
                    </ul>
                    <div className="content">
                        <Route exact path="/" component={Blog}/>
                        {/*<Route path="/blog" component={Blog}/>*/}
                        <Route path="/newpost" component={NewPost}/>
                        <Route path="/contact" component={Contact}/>
                        <Route path="/login" component={Login}/>
                        <Route path="/logout" component={Logout}/>
                    </div>
                </div>
            </HashRouter>
        );
    }
}

export default App;
