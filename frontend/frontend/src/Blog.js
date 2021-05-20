import React, {Component} from "react";
import axios from 'axios';

class Blog extends Component {
    constructor(props) {
        super(props);

        this.state = {
            posts: []
        };

        this.getBlogPosts = this.getBlogPosts.bind(this);
    }

    getBlogPosts() {
        // let url = '/api/posts/';
        let url = 'http://localhost:8000/api/posts/';
        axios.get(url)
            .then(res => {
                console.log(res);

                let posts = res.data;
                this.setState({posts});
            })
            .catch(function (error) {
                console.log('Error: ', error.message);
            })
    }

    renderPost(post, key) {
        function addHtmlNewlines(content) {
            let newContent = "";
            let objects = content.split("\n");

            for (let i = 0; i < objects.length; i++) {
                newContent += objects[i];
                newContent += "<br>";
            }

            return newContent;
        }

        return (
            <div className="blogpost" key={key}>
                <div className='post-header'>
                    <div className='titlebar'>
                        <span className='post-title'>{post.title}</span>
                        <span className='post-date'>{post.date} <i className="far fa-calendar-alt"></i></span>
                    </div>
                    <span className='post-author'><i className="fas fa-user-alt"></i> {post.author.name}</span>
                </div>
                <div className='post-content' dangerouslySetInnerHTML={{__html: addHtmlNewlines(post.content)}}/>
                <div className='post-footer'>
                    <div className='comment-link'>
                        Komentarze (0)
                    </div>
                    <div className='comment-link'>
                        Edytuj
                        Usuń
                    </div>
                </div>
            </div>
        )
    }

    renderBlogPosts() {
        const list = this.state.posts.map((post) =>
            <div key={post.id}>
                {this.renderPost(post)}
            </div>
        );

        // if (list.length === 0) {
        //     return ("Brak postów!")
        // }

        return (
            <div className="posts">
                {list}
            </div>
        )
    }

    componentDidMount() {
        this.getBlogPosts()
    }

    render() {
        return (
            <div>
                {/*<h2>Blog</h2>*/}
                {this.renderBlogPosts()}
            </div>
        );
    }
}

export default Blog;