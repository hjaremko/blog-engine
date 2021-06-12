import React, {Component} from "react";
import {
    Route,
    NavLink,
    HashRouter
} from "react-router-dom";
import Blog from "./Blog";
import NewPost from "./NewPost";
import Contact from "./Contact";

class App extends Component {
    render() {
        return (
            <HashRouter>
                <div>
                    <h1>Blog</h1>
                    <ul className="header">
                        <li><NavLink exact to="/">Wpisy</NavLink></li>
                        {/*<li><NavLink to="/blog">Blog</NavLink></li>*/}
                        <li><NavLink to="/contact">Kontakt</NavLink></li>
                        <li><NavLink to="/newpost">Dodaj post</NavLink></li>
                        <li><NavLink to="/login">Logowanie</NavLink></li>
                        <li><NavLink to="/settings">Ustawienia</NavLink></li>
                    </ul>
                    <div className="content">
                        <Route exact path="/" component={Blog}/>
                        {/*<Route path="/blog" component={Blog}/>*/}
                        <Route path="/newpost" component={NewPost}/>
                        <Route path="/contact" component={Contact}/>
                    </div>
                </div>
            </HashRouter>
        );
    }
}

export default App;
