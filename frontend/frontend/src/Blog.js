import React, {Component} from "react";
import axios from 'axios';
import {Parallax} from 'react-parallax';
import {Link} from "react-router-dom";

class Blog extends Component {
    constructor(props) {
        super(props);

        this.state = {
            posts: [],
            comments: {},
            error: '',
            loading: true,
            limit: 2,
            page: 0,
        };

        this.getBlogPosts = this.getBlogPosts.bind(this);
        this.renderBlogPosts = this.renderBlogPosts.bind(this);
    }

    renderSpinner() {
        if (!this.state.loading) {
            return;
        }

        return (
            <div className="d-flex justify-content-center">
                <div className="spinner-border" role="status" ref={loadingRef => (this.loadingRef = loadingRef)}>
                    <span className="visually-hidden">Loading...</span>
                </div>
            </div>
        )
    }

    getBlogPosts(page, limit) {
        this.setState({loading: true});

        // let url = `/api/posts/?page=${page}&limit=${limit}`;
        let url = `http://localhost:8000/api/posts?page=${page}&limit=${limit}`;
        axios.get(url)
            .then(res => {
                let newPosts = res.data;

                if (newPosts.length === 0) {
                    this.setState({loading: false});
                    return;
                }

                let posts = this.state.posts.concat(newPosts);
                // console.log('Loaded ' + posts.length + ' posts');
                this.setState({posts});
            })
            .catch((e) => {
                console.log('Error: ', e.message);
                let error = e.message;
                this.setState({error: error});
                this.setState({loading: false});
            })
    }

    getComments(postId) {
        let url = `http://localhost:8000/api/posts/comments/${postId}`;

        axios.get(url)
            .then(res => {
                let comments = res.data;
                let newComment = {[postId]: comments};
                let newComments = {...this.state.comments, ...newComment};
                this.setState({comments: newComments});
            })
            .catch((e) => {
                console.log('Error: ', e.message);
            })
    }

    renderParallax(post) {
        return (
            <div>
                <Parallax blur={0} bgImage="/img/galaxy.jfif" bgImageAlt="galaxy" strength={600}
                          className="parallax-image">
                    <div className="par-header">
                        <Link to={{
                            pathname: '/post',
                            postContent: {...post}
                        }}>{post.title}</Link>
                    </div>
                </Parallax>
            </div>
        );
    }

    expandComments(e, idx) {
        const acc = e.target;
        const panel = acc.nextElementSibling;
        if (panel.style.maxHeight) {
            panel.style.maxHeight = null;
        } else {
            panel.style.maxHeight = panel.scrollHeight + "px";
        }

        this.getComments(idx);
    }

    renderComment(comment, id) {
        return (
            <div key={id}>
                <div className='titlebar'>
                    <span className='post-author'><i className="fas fa-user-alt"></i> {comment.author.name}</span>
                    <span className='post-date'>{comment.date} <i className="far fa-calendar-alt"></i></span>
                </div>
                {comment.content}
            </div>
        )
    }

    renderComments(idx) {
        let comment = this.state.comments[idx];

        if (comment == null) {
            return;
        }

        return comment.map((comment) => {
            return (
                <div key={comment.id}>
                    <div>
                        {this.renderComment(comment, comment.id)}
                    </div>
                </div>
            )
        });
    }

    showComments(idx) {
        let c = this.state.comments[idx];
        let commentAmount = c == null ? 0 : c.length;

        return (
            <div>
                <button className="accordion" onClick={(e) => this.expandComments(e, idx)}>Komentarze
                    ({commentAmount})
                </button>
                <div className="comment-panel">
                    {this.renderComments(idx)}
                </div>
            </div>
        )
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
                    {this.renderParallax(post)}
                    <div className='titlebar'>
                        <span className='post-author'><i className="fas fa-user-alt"></i> {post.author.name}</span>
                        <span className='post-date'>{post.date} <i className="far fa-calendar-alt"></i></span>
                    </div>
                </div>
                <div className='post-content' dangerouslySetInnerHTML={{__html: addHtmlNewlines(post.content)}}/>
                {this.showComments(key)}
            </div>
        )
    }

    renderBlogPosts() {
        if (this.state.error !== '') {
            return (
                <div className="posts">
                    Error loading posts, sorry!
                </div>
            )
        }

        const list = this.state.posts.map((post) => {
            return (
                <div key={post.id}>
                    {this.renderPost(post, post.id)}
                </div>
            )
        });

        if (list.length === 0) {
            return ("Brak postów!")
        }

        return (
            <div className="posts">
                {list}
            </div>
        )
    }

    handleObserver(entities, observer) {
        if (this.state.loading) {
            this.setState({page: this.state.page + 1})
            this.getBlogPosts(this.state.page, this.state.limit)
            this.loadComments()
        }
    }

    componentDidMount() {
        this.getBlogPosts(this.state.page, this.state.limit)

        const options = {
            root: null,
            rootMargin: "0px",
            threshold: 1.0
        };

        let observer = new IntersectionObserver(
            this.handleObserver.bind(this),
            options
        );

        observer.observe(this.loadingRef);
    }

    render() {
        return (
            <div>
                {this.renderBlogPosts()}
                {this.renderSpinner()}
            </div>
        );
    }

    loadComments() {
        this.state.posts.forEach((post) => {
            this.getComments(post.id);
        });
    }
}

export default Blog;